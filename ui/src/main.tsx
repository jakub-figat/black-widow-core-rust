import React from "react";
import ReactDOM from "react-dom/client";
import router from "./src/router";
import { RouterProvider } from "react-router-dom";
import { Provider } from "react-redux";
import { store } from "./src/store";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Provider store={store}>
      <RouterProvider router={router} />
    </Provider>
  </React.StrictMode>
);
