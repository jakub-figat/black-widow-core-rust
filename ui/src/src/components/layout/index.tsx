import { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

const Layout = ({ children }: Props) => {
  return (
    <div>
      <header>
        <h1>My App</h1>
      </header>
      <main>{children}</main>
    </div>
  );
};

export { Layout };
