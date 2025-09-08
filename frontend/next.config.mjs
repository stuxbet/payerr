/** @type {import('next').NextConfig} */


import path from 'path';
import { fileURLToPath } from 'url';

const nextConfig = {
  reactStrictMode: true,
  experimental: {
    typedRoutes: true
  },
  webpack: (config) => {
  const __dirname = path.dirname(fileURLToPath(import.meta.url));
  config.resolve.alias['@'] = path.resolve(__dirname);
  // Avoid bundling optional server-only pretty printer pulled by pino via walletconnect
  config.resolve.alias['pino-pretty'] = false;
    return config;
  }
};

export default nextConfig;
