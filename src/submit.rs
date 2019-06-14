use crate::{dir, util};
use evscode::{E, R};
use std::time::Duration;
use unijudge::RejectionCause;

#[evscode::command(title = "ICIE Submit", key = "alt+f12")]
fn send() -> R<()> {
	let _status = crate::STATUS.push("Submitting");
	let (_, report) = crate::test::view::manage::COLLECTION.get_force(None)?;
	if report.runs.iter().any(|test| !test.success()) {
		return Err(E::error("some tests failed, submit aborted"));
	}
	let code = util::fs_read_to_string(dir::solution()?)?;
	let manifest = crate::manifest::Manifest::load()?;
	let url = manifest.task_url.ok_or_else(|| E::error("this folder was not initialized with Alt+F11, submit aborted"))?;
	let (sess, url) = crate::net::connect(&url)?;
	let langs = {
		let _status = crate::STATUS.push("Querying languages");
		sess.run(|sess| sess.contest(&url.contest)?.task(&url.task)?.languages())?
	};
	let good_langs = ["C++", "GNU G++17 7.3.0", "C++14 (clang 4.0)"];
	let lang = langs
		.iter()
		.find(|lang| good_langs.contains(&lang.name.as_str()))
		.ok_or_else(|| E::error("site does not support C++"))?;
	let submit_id = {
		let _status = crate::STATUS.push("Querying submit id");
		sess.run(|sess| sess.contest(&url.contest)?.task(&url.task)?.submit(lang, &code))?
	};
	track(sess, url, submit_id)?;
	Ok(())
}

fn track(sess: crate::net::Session, url: unijudge::TaskUrl, id: String) -> R<()> {
	let _status = crate::STATUS.push("Tracking");
	let sleep_duration = Duration::from_millis(500);
	let progress = evscode::Progress::new().title(format!("Tracking submit #{}", id)).show();
	let mut last_verdict = None;
	let verdict = loop {
		let submissions = {
			let _status = crate::STATUS.push("Tracking...");
			sess.run(|sess| sess.contest(&url.contest)?.task(&url.task)?.submissions())?
		};
		let submission = submissions.into_iter().find(|subm| subm.id == id).unwrap();
		let should_send = match &submission.verdict {
			unijudge::Verdict::Pending { .. } => false,
			_ => true,
		};
		if should_send {
			break submission.verdict;
		} else if Some(&submission.verdict) != last_verdict.as_ref() {
			progress.message(fmt_verdict(&submission.verdict));
			last_verdict = Some(submission.verdict);
		}
		std::thread::sleep(sleep_duration);
	};
	progress.end();
	evscode::Message::new(fmt_verdict(&verdict)).build().spawn();
	Ok(())
}

fn fmt_verdict(verdict: &unijudge::Verdict) -> String {
	let mut message = String::new();
	match verdict {
		unijudge::Verdict::Scored { score, max, cause, test } => {
			message += &format!("Scored {}", score);
			message += &max.map(|max| format!(" out of {}", max)).unwrap_or_default();
			message += fmt_cause_withtest(&cause, &test);
			message += &fmt_testid(&test);
		},
		unijudge::Verdict::Accepted => {
			message += "Accepted";
		},
		unijudge::Verdict::Rejected { cause, test } => {
			message += "Rejected";
			message += fmt_cause_withtest(&cause, &test);
			message += &fmt_testid(&test);
		},
		unijudge::Verdict::Pending { test } => {
			message += "Pending";
			message += &fmt_testid(&test);
		},
		unijudge::Verdict::Skipped => {
			message += "Skipped";
		},
	};
	message
}

fn fmt_cause_withtest(cause: &Option<RejectionCause>, test: &Option<String>) -> &'static str {
	match (cause, test) {
		(Some(RejectionCause::WrongAnswer), _) => " due to a Wrong Answer",
		(Some(RejectionCause::RuntimeError), _) => " due to a Runtime Error",
		(Some(RejectionCause::TimeLimitExceeded), _) => " due to a Time Limit Exceeded",
		(Some(RejectionCause::MemoryLimitExceeded), _) => " due to a Memory Limit Exceeded",
		(Some(RejectionCause::RuleViolation), _) => " due to a Rule Violation",
		(Some(RejectionCause::SystemError), _) => " due to a System Error",
		(Some(RejectionCause::CompilationError), _) => " due to a Compilation Error",
		(None, Some(_)) => " failing",
		(None, None) => "",
	}
}

fn fmt_testid(test: &Option<String>) -> String {
	test.as_ref().map(|test| format!(" on {}", test)).unwrap_or_default()
}
