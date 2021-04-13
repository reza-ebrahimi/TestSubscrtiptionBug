import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";
import { ApolloProvider } from "@apollo/client";
import {
  ApolloClient,
  InMemoryCache,
  split,
  HttpLink,
  ApolloLink,
} from "@apollo/client";
import { getMainDefinition } from "@apollo/client/utilities";
import { WebSocketLink } from "@apollo/client/link/ws";

var Network = {
  httpLink: function () {
    return new HttpLink({
      uri: "http://localhost:8002",
    });
  },

  wsLink: function () {
    return new WebSocketLink({
      uri: `ws://localhost:8001`,
      options: {
        reconnect: true,
      },
    });
  },

  terminatingLink: function () {
    return split(
      ({ query }) => {
        const { kind, operation } = getMainDefinition(query);
        return (
          kind === "OperationDefinition" &&
          (operation === "subscription" || operation === "subscription")
        );
      },
      this.wsLink(),
      this.httpLink()
    );
  },

  client: function () {
    return new ApolloClient({
      link: ApolloLink.from([this.terminatingLink()]),
      cache: new InMemoryCache(),
    });
  },
};

ReactDOM.render(
  <React.StrictMode>
    <ApolloProvider client={Network.client()}>
      <App />
    </ApolloProvider>
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
