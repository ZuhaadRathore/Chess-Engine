/// <reference types="vite/client" />

// SVG imports as URL strings
declare module '*.svg' {
  const content: string;
  export default content;
}

// SVG imports as React components (using ?react suffix)
declare module '*.svg?react' {
  import { FunctionComponent, SVGProps } from 'react';
  const content: FunctionComponent<SVGProps<SVGSVGElement>>;
  export default content;
}

// PNG/JPG imports (if needed later)
declare module '*.png' {
  const content: string;
  export default content;
}

declare module '*.jpg' {
  const content: string;
  export default content;
}
