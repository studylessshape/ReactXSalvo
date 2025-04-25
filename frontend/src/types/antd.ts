import { SeedToken } from "antd/es/theme/internal";
import { MapToken } from "antd/es/theme/interface";

type ThemeAlgorithm = (token: SeedToken) => MapToken;

export type { ThemeAlgorithm }