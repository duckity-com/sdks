import type { Metadata } from "next";
import { Montserrat } from "next/font/google";
import "./globals.css";
import { cn } from "@/lib/utils";

const montserrat = Montserrat({ subsets: ["latin"], variable: "--font-sans" });

export const metadata: Metadata = {
  title: "@duckity/react",
  description:
    "An example React application to demonstrate @duckity/react's usage.",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html
      lang="en"
      className={cn("dark", "h-full", "antialiased", "font-sans", montserrat.variable)}
    >
      <body className="min-h-full flex flex-col">{children}</body>
    </html>
  );
}
