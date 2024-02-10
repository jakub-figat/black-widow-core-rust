import { createBrowserRouter } from "react-router-dom";

import { ProtectedRoute } from "./components/protected-route";
import { routes } from "../config/routes";

import Home from "../views/home";
import LoginView from "../views/login";

const router = createBrowserRouter([
  {
    path: routes.home,
    element: (
      <ProtectedRoute>
        <Home />
      </ProtectedRoute>
    ),
  },
  {
    path: routes.login,
    element: <LoginView />,
  },
]);

export default router;
