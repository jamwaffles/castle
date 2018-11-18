import * as React from "react";

const Container = ({ children }: { children: any }) => (
  <html>
    <head>
      <script src="/assets/app.js" defer />
      <link rel="stylesheet" href="/assets/style.css" />
    </head>

    <body>
      <main id="app">{children}</main>
    </body>
  </html>
);

export default Container;
