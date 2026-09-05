import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  turbopack: {
    root: "../../",
    resolveExtensions: [".ts", ".tsx", ".js", ".jsx"],
  },
  allowedDevOrigins: ["127.0.0.1"]
};

export default nextConfig;
