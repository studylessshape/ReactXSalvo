import { get, post } from "./request";

export async function hello(name: string | String | null = null) {
    let params = null;
    if (name != null) {
        params = { name: name };
    }
    return await get("/api/hello", params);
}
