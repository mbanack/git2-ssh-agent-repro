// see comments after main() for output in various configurations

fn main() {
    let git_root = shellexpand::tilde("~/sa").to_string();
    match git2::Repository::open_ext(
            git_root,
            git2::RepositoryOpenFlags::NO_SEARCH,
            &[] as &[&std::ffi::OsStr]) {
        Err(e) => panic!("Failed to open_ext: {e:?}"),
        Ok(gr) => {
            let mut gr_remote = gr.find_remote("origin")
                .expect("repo: failed to lookup remote origin");

            let mut cb = git2::RemoteCallbacks::new();
            cb.credentials(move |_url, username_from_url, _allowed_types| {
                // INFO: this hits if enabled
                //panic!("credentials callback called");

                let user = match username_from_url {
                    Some(u) => u.to_string(),
                    None => std::env::var("USER").expect("no env{USER} set"),
                };
                let x = git2::Cred::ssh_key_from_agent(&user);
                if let Err(e) = x {
                    panic!("Failed to lookup cred from ssh agent: {e:?}");
                }
                x
            });

            let fetch_with_remoteconn = false;
            if fetch_with_remoteconn {
                let mut rc: git2::RemoteConnection = gr_remote.connect_auth(
                    git2::Direction::Fetch, Some(cb), None).unwrap();

                /*
                 * apparently all of this is not even hit:
                assert!(rc.connected());
                println!("RemoteConnection connected");

                let mut fo_cb = git2::RemoteCallbacks::new();
                fo_cb.credentials(move |_url, username_from_url, _allowed_types| {
                    // INFO: this hits if enabled
                    //panic!("credentials callback called");

                    let user = match username_from_url {
                        Some(u) => u.to_string(),
                        None => std::env::var("USER").expect("no env{USER} set"),
                    };
                    let x = git2::Cred::ssh_key_from_agent(&user);
                    if let Err(e) = x {
                        panic!("Failed to lookup cred from ssh agent: {e:?}");
                    }
                    x
                });
                let mut fo = git2::FetchOptions::new();
                fo.remote_callbacks(fo_cb);

                let rcr = rc.remote();

                println!("fetching with RemoteConnection.remote() and callbacks.credentials");
                match rcr.fetch(&["main"], Some(&mut fo), None) {
                    Err(e) => {
                        println!("Failed to fetch with RemoteConnection.remote(): {e:?}");
                    }
                    Ok(_) => {
                        println!("Fetch succeeded with RemoteConnection.remote()");
                    }
                }
                */
            } else {
                println!("fetching with Repository.remote() and callbacks.credentials");
                let mut fo = git2::FetchOptions::new();
                fo.remote_callbacks(cb);
                gr_remote.fetch(&["main"], Some(&mut fo), None).unwrap();
            }
        }
    }
}

/*
 * With panic!(...) in cb.credentials callback,
 * and fetch_with_remoteconn = true;
 *
thread 'main' panicked at src/main.rs:15:17:
credentials callback called
stack backtrace:
   0: rust_begin_unwind
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/panicking.rs:74:14
   2: git2_ssh_agent_repro::main::{{closure}}
             at ./src/main.rs:15:17
   3: <alloc::boxed::Box<F,A> as core::ops::function::FnMut<Args>>::call_mut
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/alloc/src/boxed.rs:2157:9
   4: git2::remote_callbacks::credentials_cb::{{closure}}
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/git2-0.19.0/src/remote_callbacks.rs:333:13
   5: std::panicking::try::do_call
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/std/src/panicking.rs:554:40
   6: ___rust_try
   7: std::panicking::try
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/std/src/panicking.rs:518:19
   8: std::panic::catch_unwind
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/std/src/panic.rs:345:14
   9: git2::panic::wrap
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/git2-0.19.0/src/panic.rs:13:11
  10: git2::remote_callbacks::credentials_cb
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/git2-0.19.0/src/remote_callbacks.rs:315:18
  11: request_creds
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/transports/ssh_libssh2.c:394:11
  12: _git_ssh_setup_conn
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/transports/ssh_libssh2.c:828:16
  13: ssh_uploadpack_ls
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/transports/ssh_libssh2.c:916:9
  14: _ssh_action
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/transports/ssh_libssh2.c:972:11
  15: git_smart__connect
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/transports/smart.c:164:15
  16: git_remote_connect_ext
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/remote.c:963:15
  17: git_remote_connect
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libgit2-sys-0.17.0+1.8.1/libgit2/src/libgit2/remote.c:1002:9
  18: git2::remote::Remote::connect_auth
             at /Users/mx/.cargo/registry/src/index.crates.io-6f17d22bba15001f/git2-0.19.0/src/call.rs:13:39
  19: git2_ssh_agent_repro::main
             at ./src/main.rs:33:54
  20: core::ops::function::FnOnce::call_once
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/ops/function.rs:250:5
 */

/*
 * With NO panic!(...) in cb.credentials callback,
 * and fetch_with_remoteconn = true;
thread 'main' panicked at src/main.rs:33:61:
called `Result::unwrap()` on an `Err` value: Error { code: -16, klass: 23, message: "authentication required but no callback set" }
stack backtrace:
   0: rust_begin_unwind
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/panicking.rs:74:14
   2: core::result::unwrap_failed
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/result.rs:1679:5
   3: core::result::Result<T,E>::unwrap
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/result.rs:1102:23
   4: git2_ssh_agent_repro::main
             at ./src/main.rs:32:54
   5: core::ops::function::FnOnce::call_once
             at /rustc/730d5d4095a264ef5f7c0a0781eea68c15431d45/library/core/src/ops/function.rs:250:5
 */

/*
 * With NO panic!(...) in cb.credentials callback,
 * and fetch_with_remoteconn = false;
 fetching with Repository.remote() and callbacks.credentials
Failed to fetch with Repository.remote: Error { code: -16, klass: 23, message: "authentication required but no callback set" }
 */
