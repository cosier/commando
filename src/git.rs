use git2;
use std::result::{Result};
use std::error::Error;
use utils::errors::{CommandoResult};

pub fn with_authentication<T, F>(url: &str, cfg: &git2::Config, mut f: F)
                             -> CommandoResult<T>
    where F: FnMut(&mut git2::Credentials) -> CommandoResult<T>
{
    // Prepare the authentication callbacks.
    //
    // We check the `allowed` types of credentials, and we try to do as much as
    // possible based on that:
    //
    // * Prioritize SSH keys from the local ssh agent as they're likely the most
    //   reliable. The username here is prioritized from the credential
    //   callback, then from whatever is configured in git itself, and finally
    //   we fall back to the generic user of `git`.
    //
    // * If a username/password is allowed, then we fallback to git2-rs's
    //   implementation of the credential helper. This is what is configured
    //   with `credential.helper` in git, and is the interface for the OSX
    //   keychain, for example.
    //
    // * After the above two have failed, we just kinda grapple attempting to
    //   return *something*.
    let mut cred_helper = git2::CredentialHelper::new(url);
    cred_helper.config(cfg);
    let mut cred_error = false;
    let ret = f(&mut |url, username, allowed| {
        let creds = if allowed.contains(git2::SSH_KEY) {
            let user = username.map(|s| s.to_string())
                .or_else(|| cred_helper.username.clone())
                .unwrap_or("git".to_string());
            git2::Cred::ssh_key_from_agent(user.as_slice())
        } else if allowed.contains(git2::USER_PASS_PLAINTEXT) {
            git2::Cred::credential_helper(cfg, url, username)
        } else if allowed.contains(git2::DEFAULT) {
            git2::Cred::default()
        } else {
            Err(git2::Error::from_str("no authentication available"))
        };
        cred_error = creds.is_err();
        creds
    });
    if cred_error {
        ret.unwrap().chain_error(|| {
            human("Failed to authenticate when downloading repository")
        })
    } else {
        ret
    }
}

pub fn fetch(repo: &git2::Repository, url: &str,
             refspec: &str) -> CommandoResult<()> {
    // Create a local anonymous remote in the repository to fetch the url

    with_authentication(url, &try!(repo.config()), |f| {
        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(|a, b, c| f(a, b, c));
        let mut remote = try!(repo.remote_anonymous(url.as_slice(),
                                                    Some(refspec)));
        try!(remote.add_fetch("refs/tags/*:refs/tags/*"));
        remote.set_callbacks(&mut cb);
        try!(remote.fetch(&["refs/tags/*:refs/tags/*", refspec], None, None));
        Ok(())
    })
}

fn human(s: &str) {
    println!("{}", s);
}
