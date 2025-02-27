export async function post(
    path: string | URL,
    data?: BodyInit | null,
): Promise<Response> {
    return await fetch(path, {
        body: data,
    });
}

export async function get(
    path: string | URL,
    params: URLSearchParams | any | null = null,
): Promise<Response> {
    let query;
    if (params === null) {
        query = "";
    } else {
        if (params instanceof URLSearchParams) {
            query = '?' + params;
        } else {
            query = '?' + new URLSearchParams(params);
        }
    }
    return await fetch(path + query);
}
