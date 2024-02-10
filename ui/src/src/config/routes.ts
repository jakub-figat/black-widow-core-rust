export const routes = {
  home: "/",
  login: "/login",
} as const;

export type AppRoutes = keyof typeof routes;
