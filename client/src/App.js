import React, { useState, useEffect } from "react";
import { gql, useSubscription } from "@apollo/client";

const SUBSCRIPTION = gql`
  subscription {
    stat
  }
`;

const TestComponent = () => {
  const { loading, error, data } = useSubscription(SUBSCRIPTION);

  useEffect(() => {
    return () => {
      // unsubscribe here
    };
  }, []);

  if (error) {
    return `Query Error! ${error.message}`;
  }
  if (loading) {
    return null;
  }

  return <div>TestComponent {data.stat}</div>;
};

function App() {
  const [state, setState] = useState(false);
  return (
    <div>
      <button
        onClick={() => {
          setState(!state);
        }}
      >
        {state ? "Unmount" : "Mount"}
      </button>
      {state && <TestComponent />}
    </div>
  );
}

export default App;
