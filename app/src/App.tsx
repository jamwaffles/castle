import * as React from "react";
import { Store } from "redux";
import { Switch, Route } from "react-router";
import { Provider } from "react-redux";
import { hot } from "react-hot-loader";

import Home from "./pages/Home";

export const routes: any[] = [{ path: "/", component: Home }];

const App = ({ store }: { store: Store }) => (
  <Provider store={store}>
    <div>
      <Switch>
        {routes.map(route => (
          <Route key={route.path} {...route} />
        ))}
      </Switch>
    </div>
  </Provider>
);

export default hot(module)(App);
