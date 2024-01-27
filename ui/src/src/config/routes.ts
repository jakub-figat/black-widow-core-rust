export const routes = {
  home: "/",
} as const;

export type AppRoutes = keyof typeof routes;
