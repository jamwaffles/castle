import * as React from "react";
import * as ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";

import "./styles/style.less";

import App from "./App";
import { createStore } from "./store";

console.log("Entry point");

const store = createStore();

const container = document.getElementById("app");

ReactDOM.render(
  <BrowserRouter>
    <App store={store} />
  </BrowserRouter>,
  container
);
