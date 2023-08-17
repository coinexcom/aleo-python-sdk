from .zero_knowledge_proof import ZeroKnowledgeProof
from ._model_transpiler import _get_model_transpiler
import os, time, subprocess, psutil
from sklearn.base import BaseEstimator
from typing import Optional
from numpy.typing import ArrayLike

class LeoTranspiler:

    def __init__(self, 
             model: BaseEstimator, 
             validation_data: Optional[ArrayLike] = None, 
             model_as_input: bool = False, 
             ouput_model_hash: Optional[str] = None):
        """Initializes the LeoTranspiler with the given parameters.

        Parameters
        ----------
        model : BaseEstimator
            The ML model to transpile.
        validation_data : tuple of array_like, optional
            Data to evaluate the numerical stability of the circuit. The model will not be trained on this data.
        model_as_input : bool, optional
            If True, the model's weights and biases are treated as circuit input rather than being hardcoded.
        output_model_hash : str, optional
            If provided, the circuit returns the hash of the model's weights and biases. Possible values are ... (todo)
        """

        self.model = model
        self.validation_data = validation_data
        self.model_as_input = model_as_input
        self.ouput_model_hash = ouput_model_hash

        self.transpilation_result = None
        self.leo_program_stored = False

    def store_leo_program(self, path, project_name):
        """
        Store the Leo program to a file.

        Parameters
        ----------
        path : str
            The path to the file to store the Leo program in.

        Returns
        -------
        None
        """ 

        self.model_transpiler = _get_model_transpiler(self.model, self.validation_data)

        # Check numeric stability for model and data and get number range
        self.model_transpiler._numbers_get_leo_type_and_fixed_point_scaling_factor()

        if self.transpilation_result is None:
            print("Transpiling model...")
            self.transpilation_result = self.model_transpiler.transpile(project_name) # todo check case when project name changes

        self.project_dir = os.path.join(path, project_name)
        src_folder_dir = os.path.join(self.project_dir, "src")
        leo_file_dir = os.path.join(src_folder_dir, "main.leo")

        # Make sure path exists
        os.makedirs(src_folder_dir, exist_ok=True)

        with open(leo_file_dir, "w") as f:
            f.write(self.transpilation_result)

        program_json = self._get_program_json(project_name)
        program_json_file_dir = os.path.join(self.project_dir, "program.json")
        with open(program_json_file_dir, "w") as f:
            f.write(program_json)
        
        environment_file = self._get_environment_file() # todo option to pass private key
        environment_file_dir = os.path.join(self.project_dir, ".env")
        with open(environment_file_dir, "w") as f:
            f.write(environment_file)

        self.leo_program_stored = True
        print("Leo program stored")

    def prove(self, inputs_decimal):
        """
        Prove the model output for a given input.

        Parameters
        ----------
        input : array_like
            The input for which to prove the output.

        Returns
        -------
        ZeroKnowledgeProof
            The zero-knowledge proof for the given input.
        """
        if not self.leo_program_stored:
            raise Exception("Leo program not stored")
        
        circuit_inputs_fixed_point = self.model_transpiler.generate_input(inputs_decimal)

        # TODO: here we need to do the FFI call or CLI call for leo/snarkVM execute
        circuit_output, proof_value = None, None
        command = ['leo', 'run', 'main'] + circuit_inputs_fixed_point
        directory = self.project_dir

        # Start Leo program
        start = time.time()
        process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=directory)

        while process.poll() is None:
            try:
                time.sleep(0.1)
            except psutil.NoSuchProcess:
                break

        end = time.time()

        # Get the output
        stdout, stderr = process.communicate()
        result = stdout.decode() + stderr.decode()
        runtime = end - start

        outputs_fixed_point = []

        success = "Finished" in result
        if success:
            constraints = int(result.split("constraints")[0].split()[-1].replace(",", ""))
            # Output processing
            outputs_str = result.split("Output")[1]
            outputs_str = outputs_str.split("• ")
            for element in outputs_str:
                if element.startswith("\n"):
                    continue
                # check if is number
                if element[0].isdigit():
                    element = element.split(self.model_transpiler.leo_type)[0]
                    outputs_fixed_point.append(int(element))
        else:
            print("Error:", result)

        outputs_decimal = self.model_transpiler.convert_from_fixed_point(outputs_fixed_point)

        return ZeroKnowledgeProof(inputs_decimal, outputs_decimal, None)
    
    def _get_program_json(self, project_name):
        """
        Generate the program.json file content.

        Parameters
        ----------
        project_name : str
            The name of the project.

        Returns
        -------
        str
            The program.json file.
        """
        return f"""{{
    "program": "{project_name}.aleo",
    "version": "0.0.0",
    "description": "transpiler generated program",
    "license": "MIT"
}}"""

    def _get_environment_file(self):
        """
        Generate the environment file content.

        Returns
        -------
        str
            The environment file.
        """
        return f"""NETWORK=testnet3
PRIVATE_KEY=APrivateKey1zkpHtqVWT6fSHgUMNxsuVf7eaR6id2cj7TieKY1Z8CP5rCD
"""