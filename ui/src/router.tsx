import { createBrowserRouter } from "react-router-dom";
import { routes } from "./src/config/routes";
import Home from "./src/views/home";

const router = createBrowserRouter([
  {
    path: routes.home,
    element: <Home />,
  },
]);

export default router;
