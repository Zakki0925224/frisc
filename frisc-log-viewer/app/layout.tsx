import type { Metadata } from "next";
import { Source_Code_Pro } from "next/font/google";

const inter = Source_Code_Pro({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "frisc-log-viewer",
    description: "log viewer",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>)
{
    return (
        <html lang="en">
            <body className={inter.className}>{children}</body>
        </html>
    );
}
