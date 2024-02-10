import { SxProps } from "@mui/material";
import { Theme } from "@mui/system";

const constructSxStyles = <O extends { [key: string]: SxProps<Theme> }>(
  obj: O
) => obj;

export { constructSxStyles };
