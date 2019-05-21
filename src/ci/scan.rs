use itertools::Itertools;
use std::{
	cmp::Ordering, path::{Path, PathBuf}
};

pub fn scan_and_order(root: &Path) -> Vec<PathBuf> {
	let mut tests = scan(root);
	order(&mut tests);
	tests
}

pub fn scan(root: &Path) -> Vec<PathBuf> {
	walkdir::WalkDir::new(root)
		.follow_links(true)
		.into_iter()
		.filter_map(|e| e.ok())
		.map(|entry| entry.path().to_path_buf())
		.filter(|path| path.extension().map(|ext| ext == "in").unwrap_or(false))
		.collect()
}

pub fn order(tests: &mut Vec<PathBuf>) {
	tests.sort_by(comp_by_test_number);
}

fn comp_by_test_number(lhs: &std::path::PathBuf, rhs: &std::path::PathBuf) -> Ordering {
	for grp in lhs
		.to_str()
		.unwrap()
		.chars()
		.group_by(|c| c.is_numeric())
		.into_iter()
		.zip_longest(rhs.to_str().unwrap().chars().group_by(|c| c.is_numeric()).into_iter())
	{
		match grp {
			itertools::EitherOrBoth::Both((isdig, lgrp), (_, rgrp)) => {
				let grp_compr = if isdig {
					let lnum: i64 = lgrp.collect::<String>().parse().unwrap();
					let rnum: i64 = rgrp.collect::<String>().parse().unwrap();
					lnum.cmp(&rnum)
				} else {
					lgrp.cmp(rgrp)
				};
				if grp_compr != Ordering::Equal {
					return grp_compr;
				}
			},
			itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
			itertools::EitherOrBoth::Right(_) => return Ordering::Less,
		}
	}
	Ordering::Equal
}