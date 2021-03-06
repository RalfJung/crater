use crate::crates::{Crate, GitHubRepo};
use crate::experiments::Experiment;
use crate::prelude::*;
use crate::results::{EncodedLog, ReadResults, TestResult};
use crate::toolchain::Toolchain;
use std::collections::HashMap;

#[derive(Default)]
struct DummyData {
    shas: HashMap<GitHubRepo, String>,
    logs: HashMap<(Crate, Toolchain), EncodedLog>,
    results: HashMap<(Crate, Toolchain), TestResult>,
}

#[derive(Default)]
pub struct DummyDB {
    experiments: HashMap<String, DummyData>,
}

impl DummyDB {
    fn get_data(&self, ex: &Experiment) -> Fallible<&DummyData> {
        Ok(self
            .experiments
            .get(&ex.name)
            .ok_or_else(|| err_msg(format!("missing experiment {}", ex.name)))?)
    }

    pub fn add_dummy_sha(&mut self, ex: &Experiment, repo: GitHubRepo, sha: String) {
        self.experiments
            .entry(ex.name.to_string())
            .or_insert_with(DummyData::default)
            .shas
            .insert(repo, sha);
    }

    pub fn add_dummy_log(&mut self, ex: &Experiment, krate: Crate, tc: Toolchain, log: EncodedLog) {
        self.experiments
            .entry(ex.name.to_string())
            .or_insert_with(DummyData::default)
            .logs
            .insert((krate, tc), log);
    }

    pub fn add_dummy_result(
        &mut self,
        ex: &Experiment,
        krate: Crate,
        tc: Toolchain,
        res: TestResult,
    ) {
        self.experiments
            .entry(ex.name.to_string())
            .or_insert_with(DummyData::default)
            .results
            .insert((krate, tc), res);
    }
}

impl ReadResults for DummyDB {
    fn load_all_shas(&self, ex: &Experiment) -> Fallible<HashMap<GitHubRepo, String>> {
        Ok(self.get_data(ex)?.shas.clone())
    }

    fn load_log(
        &self,
        ex: &Experiment,
        toolchain: &Toolchain,
        krate: &Crate,
    ) -> Fallible<Option<EncodedLog>> {
        Ok(self
            .get_data(ex)?
            .logs
            .get(&(krate.clone(), toolchain.clone()))
            .cloned())
    }

    fn load_test_result(
        &self,
        ex: &Experiment,
        toolchain: &Toolchain,
        krate: &Crate,
    ) -> Fallible<Option<TestResult>> {
        Ok(self
            .get_data(ex)?
            .results
            .get(&(krate.clone(), toolchain.clone()))
            .cloned())
    }
}
