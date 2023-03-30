import React from "react";

export function SignInPrompt({ onClick }) {
  return (
    <main className="container">
      <div className="row my-3">
        <div className="col-12 text-center">
          <h1 className="mt-3">Welcome to NEAR XCC-CALCULATOR!</h1>
          <p>
            Learn to make cross-contract calls on Near blockchain the easy way.
          </p>
          <button className="btn btn-primary mt-3" onClick={onClick}>
            Sign in with NEAR Wallet
          </button>
        </div>
      </div>
    </main>
  );
}

export function SignOutButton({ accountId, onClick }) {
  return (
    <button className="btn btn-danger my-3" onClick={onClick}>
      Sign out {accountId}
    </button>
  );
}
