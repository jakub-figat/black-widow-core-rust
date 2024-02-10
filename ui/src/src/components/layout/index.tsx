import { ReactNode } from "react";
import { Nabar } from "../navbar";
import { Box } from "@mui/material";
import { constructSxStyles } from "../../utils/construct-sx-styles";
import { MAX_LAYOUT_WIDTH } from "../../config/consts";

interface Props {
  children: ReactNode;
}

const Layout = ({ children }: Props) => {
  const styles = constructSxStyles({
    layoutWrapper: {
      width: "100%",
      maxWidth: MAX_LAYOUT_WIDTH,
      margin: "0 auto",
      px: 2,
    },
  });
  return (
    <Box sx={styles.layoutWrapper}>
      <Nabar />
      {children}
    </Box>
  );
};

export { Layout };
