import React from "react";
import ReactDOM from "react-dom/client";
import router from "./src/router";
import { RouterProvider } from "react-router-dom";
import { Provider } from "react-redux";
import { store } from "./src/store";
import { CssBaseline, ThemeProvider, createTheme } from "@mui/material";

const theme = createTheme({});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Provider store={store}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <RouterProvider router={router} />
      </ThemeProvider>
    </Provider>
  </React.StrictMode>
);
