// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    types::{CurrentAleo, CurrentNetwork, ProcessNative, ProgramIDNative},
    util::os_rng,
    Authorization, Execution, Fee, Field, Identifier, MicroCredits, PrivateKey, Program, ProgramID,
    ProvingKey, RecordPlaintext, Response, Trace, Value,
};

use indexmap::IndexMap;
use pyo3::prelude::*;
use snarkvm::algorithms::snark::varuna::VarunaVersion;
use snarkvm::prelude::{ConsensusVersion, InclusionVersion, Stack};
use std::sync::Arc;

/// The Aleo process type.
#[pyclass]
pub struct Process(ProcessNative);

fn execution_stacks_for_execution(
    process: &ProcessNative,
    execution: &Execution,
) -> anyhow::Result<IndexMap<ProgramIDNative, Arc<Stack<CurrentNetwork>>>> {
    let mut execution_stacks = IndexMap::new();
    for transition in execution.transitions() {
        execution_stacks.insert(
            *transition.program_id(),
            process.get_stack(transition.program_id())?,
        );
    }
    Ok(execution_stacks)
}

#[pymethods]
impl Process {
    /// Initializes a new process.
    #[staticmethod]
    fn load() -> anyhow::Result<Self> {
        ProcessNative::load().map(Self)
    }

    /// Adds a new program to the process
    fn add_program(&self, program: &Program) -> anyhow::Result<()> {
        self.0.lock().add_program(&**program).map_err(Into::into)
    }

    /// Returns true if the process contains the program with the given ID.
    fn contains_program(&self, program_id: &ProgramID) -> bool {
        self.0.contains_program(program_id)
    }

    /// Returns the proving key for the given program ID and function name.
    fn get_proving_key(
        &self,
        program_id: ProgramID,
        function_name: Identifier,
    ) -> anyhow::Result<ProvingKey> {
        self.0
            .get_proving_key(program_id, function_name)
            .map(ProvingKey::from)
    }

    /// Inserts the given proving key, for the given program ID and function name.
    fn insert_proving_key(
        &self,
        program_id: &ProgramID,
        function_name: &Identifier,
        proving_key: ProvingKey,
    ) -> anyhow::Result<()> {
        self.0
            .insert_proving_key(program_id, function_name, proving_key.into())
    }

    /// Authorizes a call to the program function for the given inputs.
    fn authorize(
        &self,
        private_key: &PrivateKey,
        program_id: ProgramID,
        function_name: Identifier,
        inputs: Vec<Value>,
    ) -> anyhow::Result<Authorization> {
        self.0
            .authorize::<CurrentAleo, _>(
                private_key,
                program_id,
                function_name,
                inputs.into_iter(),
                &mut os_rng(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Authorizes the fee given the credits record, the fee amount (in microcredits), and the deployment or execution ID.
    fn authorize_fee_private(
        &self,
        private_key: &PrivateKey,
        credits: RecordPlaintext,
        base_fee: MicroCredits,
        deployment_or_execution_id: Field,
        priority_fee: Option<MicroCredits>,
    ) -> anyhow::Result<Authorization> {
        self.0
            .authorize_fee_private::<CurrentAleo, _>(
                private_key,
                credits.into(),
                base_fee.into(),
                priority_fee.map(Into::into).unwrap_or(0),
                deployment_or_execution_id.into(),
                &mut os_rng(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Authorizes the fee given the the fee amount (in microcredits) and the deployment or execution ID.
    fn authorize_fee_public(
        &self,
        private_key: &PrivateKey,
        base_fee: MicroCredits,
        deployment_or_execution_id: Field,
        priority_fee: Option<MicroCredits>,
    ) -> anyhow::Result<Authorization> {
        self.0
            .authorize_fee_public::<CurrentAleo, _>(
                private_key,
                base_fee.into(),
                priority_fee.map(Into::into).unwrap_or(0),
                deployment_or_execution_id.into(),
                &mut os_rng(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Executes the given authorization.
    fn execute(&self, authorization: Authorization) -> anyhow::Result<(Response, Trace)> {
        self.0
            .execute::<CurrentAleo, _>(authorization.into(), &mut os_rng())
            .map(|(r, t)| (Response::from(r), Trace::from(t)))
            .map_err(Into::into)
    }

    /// Verifies the given execution is valid. Note: This does not check that the global state root exists in the ledger.
    fn verify_execution(&self, execution: &Execution) -> anyhow::Result<()> {
        let execution_stacks = execution_stacks_for_execution(&self.0, execution)?;
        ProcessNative::verify_execution(
            ConsensusVersion::V10,
            VarunaVersion::V2,
            InclusionVersion::V1,
            execution,
            &execution_stacks,
        )
        .map_err(Into::into)
    }

    /// Verifies the given fee is valid. Note: This does not check that the global state root exists in the ledger.
    fn verify_fee(&self, fee: &Fee, deployment_or_execution_id: Field) -> anyhow::Result<()> {
        self.0
            .verify_fee(
                ConsensusVersion::V10,
                VarunaVersion::V2,
                InclusionVersion::V1,
                fee,
                deployment_or_execution_id.into(),
            )
            .map_err(Into::into)
    }
}
