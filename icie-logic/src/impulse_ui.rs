use crate::Impulse;
use ci::{self, commands::list_resources::Resource, error::Error, strres::StrRes, testing::TestResult};
use std::{
	path::{Path, PathBuf}, sync::mpsc::{self, Sender}, time::Duration
};

pub struct ImpulseCiUi(pub Sender<Impulse>);

impl ci::ui::Ui for ImpulseCiUi {
	fn read_auth(&mut self, domain: &str) -> (String, String) {
		let (tx, rx) = mpsc::channel();
		self.0
			.send(Impulse::CiAuthRequest {
				domain: domain.to_owned(),
				channel: tx,
			})
			.unwrap();
		rx.recv().unwrap().unwrap()
	}

	fn track_progress(&mut self, verdict: &unijudge::Verdict, finish: bool) {
		self.0.send(Impulse::CiTrack { verdict: verdict.clone(), finish }).unwrap();
	}

	fn submit_success(&mut self, id: String) {
		self.0.send(Impulse::CiSubmitSuccess { id }).unwrap();
	}

	fn test_list(&mut self, paths: &[PathBuf]) {
		self.0
			.send(Impulse::CiTestList {
				paths: paths.iter().cloned().collect(),
			})
			.unwrap();
	}

	fn print_resource_list(&mut self, _resources: &[Resource]) {
		unimplemented!()
	}

	fn print_resource(&mut self, _data: &[u8]) {
		unimplemented!()
	}

	fn print_test(&mut self, outcome: &TestResult, timing: Option<Duration>, in_path: &Path, _output: Option<StrRes>) {
		self.0
			.send(Impulse::CiTestSingle {
				outcome: outcome.clone(),
				timing,
				in_path: in_path.to_owned(),
			})
			.unwrap();
	}

	fn print_finish_test(&mut self, success: bool) {
		self.0.send(Impulse::CiTestFinish { success }).unwrap();
	}

	fn print_finish_init(&mut self) {
		self.0.send(Impulse::CiInitFinish).unwrap();
	}

	fn print_transpiled(&mut self, _compiled: &str) {
		unimplemented!()
	}

	fn print_found_test(&mut self, _test_str: &str) {
		unimplemented!()
	}

	fn print_error(&mut self, _error: Error) {
		unimplemented!()
	}

	fn mt_generator_fail(&mut self, _i: i64) {
		unimplemented!()
	}

	fn mt_autogenerated(&mut self, _i: i64) {
		unimplemented!()
	}

	fn mt_good(&mut self, _t: Duration) {
		unimplemented!()
	}

	fn mt_bad(&mut self, _t: Duration) {
		unimplemented!()
	}

	fn mt_piece(&mut self, _result: &TestResult, _ti: Duration) {
		unimplemented!()
	}

	fn mt_piece_ignored(&mut self) {
		unimplemented!()
	}

	fn mt_piece_finish(&mut self) {
		unimplemented!()
	}

	fn warn(&mut self, _message: &str) {
		unimplemented!()
	}

	fn notice(&mut self, _message: &str) {}
}
