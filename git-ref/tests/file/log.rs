mod iter {
    mod forward {
        use bstr::B;
        use git_hash::ObjectId;
        use std::path::PathBuf;

        fn reflog_dir() -> crate::Result<PathBuf> {
            Ok(
                git_testtools::scripted_fixture_repo_read_only("make_repo_for_reflog.sh")?
                    .join(".git")
                    .join("logs"),
            )
        }
        fn reflog(name: &str) -> crate::Result<Vec<u8>> {
            Ok(std::fs::read(reflog_dir()?.join(name))?)
        }

        #[test]
        fn all_success() -> crate::Result {
            let log = reflog("HEAD")?;
            let iter = git_ref::file::log::iter::forward(&log);
            assert_eq!(iter.count(), 5, "the log as a known amount of entries");

            let mut iter = git_ref::file::log::iter::forward(&log);
            let line = iter.next().unwrap()?;
            assert_eq!(line.previous_oid(), ObjectId::null_sha1());
            assert_eq!(line.new_oid, B("134385f6d781b7e97062102c6a483440bfda2a03"));
            assert_eq!(line.message, B("commit (initial): c1"));
            assert!(iter.all(|l| l.is_ok()), "all lines parse fine");
            Ok(())
        }

        #[test]
        fn a_single_failure_does_not_abort_iteration() {
            let log_first_broken = "0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit
0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1\n";

            let mut iter = git_ref::file::log::iter::forward(log_first_broken.as_bytes());
            let err = iter.next().expect("error is not none").expect_err("the line is broken");
            assert_eq!(err.to_string(), "In line 1: \"0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000\\tcommit\" did not match '<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>'");
            assert!(iter.next().expect("a second line").is_ok(), "line parses ok");
            assert!(iter.next().is_none(), "iterator exhausted");
        }
    }
}
