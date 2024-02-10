import { ReactNode, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAppSelector } from "../../store/hooks";
import { getAuthenticated } from "../../store/auth";
import { routes } from "../../config/routes";

interface Props {
  children: ReactNode;
}

export const ProtectedRoute = ({ children }: Props) => {
  const navigate = useNavigate();

  const isAuthenticated = useAppSelector(getAuthenticated);

  useEffect(() => {
    if (isAuthenticated) return;
    navigate(routes.login);
  }, [isAuthenticated]);

  return children;
};
