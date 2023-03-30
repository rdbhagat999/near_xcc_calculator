import "regenerator-runtime/runtime";
import React, { useState } from "react";

import "./assets/global.css";

import { SignInPrompt, SignOutButton } from "./ui-components";

export default function App({ isSignedIn, contract, wallet }) {
  const [lastOperationResult, setLastOperationResult] = React.useState({
    op_1: 0,
    op_2: 0,
    result: 0,
    operator: "NONE",
  });

  const [operation, setOperation] = useState("NONE");

  const [uiPleaseWait, setUiPleaseWait] = React.useState(true);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (isSignedIn) {
      if (confirm("Are you sure?") != true) {
        console.log("action cancelled");
        return false;
      }

      const { inputOperand1, inputOperand2, inputOperation } =
        e.target.elements;

      if (inputOperation?.value == "NONE") {
        alert("Please select an operator.");
        return false;
      }

      try {
        setUiPleaseWait(true);

        const val1 = parseInt(inputOperand1.value, 10);
        const val2 = parseInt(inputOperand2.value, 10);

        await contract.performOperation(val1, val2, inputOperation.value);

        inputOperand1.value = 0;
        inputOperand2.value = 0;
        inputOperation.value = "NONE";

        const result = await contract.queryLastOperationResult();

        setLastOperationResult(result);
      } catch (error) {
        console.log(error);
        alert(error);
      } finally {
        setUiPleaseWait(false);
      }
    }
  };

  // Get blockchian state once on component load
  React.useEffect(() => {
    if (isSignedIn) {
      contract
        .queryLastOperationResult()
        .then((result) => {
          console.log(result);
          setLastOperationResult(result);
        })
        .catch(alert)
        .finally(() => {
          setUiPleaseWait(false);
        });
    }
  }, [isSignedIn]);

  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return (
      <main className="container">
        <SignInPrompt onClick={() => wallet.signIn()} />
      </main>
    );
  }

  return (
    <main className="container">
      <div className="row my-3">
        <div className="col-sm-12 text-center">
          <h1 className="mt-3">Welcome to NEAR XCC-CALCULATOR!</h1>
          <p>
            Learn to make cross-contract calls on Near blockchain the easy way.
          </p>
          <SignOutButton onClick={() => wallet.signOut()} />
          {uiPleaseWait ? <p>Please wait...</p> : ""}
        </div>
        <div className="col-sm-12">
          <form
            onSubmit={(e) => isSignedIn && !uiPleaseWait && handleSubmit(e)}
          >
            <div className="form-floating mb-3">
              <input
                type="number"
                className="form-control form-control-sm"
                id="inputOperand1"
                defaultValue={0}
              />
              <label htmlFor="inputOperand1">Operand1</label>
            </div>

            <div className="form-floating mb-3">
              <input
                type="number"
                className="form-control form-control-sm"
                id="inputOperand2"
                defaultValue={0}
              />
              <label htmlFor="inputOperand2">Operand2</label>
            </div>

            <div className="form-floating mb-3">
              <select
                id="inputOperation"
                className="form-select form-select-sm"
                aria-label=".form-select-sm"
                value={operation}
                onChange={(e) => setOperation(e?.target?.value)}
              >
                <option value="NONE">None</option>
                <option value="ADD">Add</option>
                <option value="MUL">Mul</option>
                <option value="DIV">Div</option>
              </select>
              <label htmlFor="inputOperation">Operation</label>
            </div>

            <div className="text-center">
              <button
                type="submit"
                className="btn btn-primary"
                disabled={uiPleaseWait}
              >
                Calculate
              </button>
            </div>
          </form>
        </div>
        <div className="col-sm-12 text-center">
          <div className="mt-3 p-2 rounded text-bg-dark">
            <p>Last operation</p>
            {lastOperationResult?.operator == "NONE" ? (
              <div>{"NONE"}</div>
            ) : (
              <div>
                {lastOperationResult?.op_1}{" "}
                {lastOperationResult?.operator == "ADD"
                  ? "+"
                  : lastOperationResult?.operator == "SUB"
                  ? "-"
                  : lastOperationResult?.operator == "MUL"
                  ? "*"
                  : "/"}{" "}
                {lastOperationResult?.op_2} = {lastOperationResult?.result}
              </div>
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
