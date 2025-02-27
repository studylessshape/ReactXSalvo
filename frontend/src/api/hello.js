import { get, post } from "./request";

export async function hello(name = null) {
    let params = null;
    if (name != null) {
        const obj = { name: name };
        params = new URLSearchParams(obj);
    }
    return await get("/api/hello", params);
}
