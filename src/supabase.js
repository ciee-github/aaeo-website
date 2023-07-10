export function create_client(url, anon_key) {
    window.supabase_client = supabase.createClient(url, anon_key);
}
export async function sign_in(email) {
    const { error } = await window.supabase_client.auth.signInWithOtp({
        email,
        options: {
            emailRedirectTo: "https://australasianeconolympiad.org/schedule"
        }
    });
    if (error) throw error;
}
export async function get_session() {
    const { data, error } = await window.supabase_client.auth.getSession()
    if (error) throw error;

    return {
        email: data.session.user.email,
        jwt: data.session.access_token,
    };
}
export async function fetch_schedule() {
    const { data: contents, error: contents_err } = await window.supabase_client
                                        .from("schedule")
                                        .select("key, value")
                                        .eq("key", "contents");
    if (contents_err) throw contents_err;
    const { data: banner, error: banner_err } = await window.supabase_client
                                        .from("schedule")
                                        .select("key, value")
                                        .eq("key", "banner");
    if (banner_err) throw banner_err;

    // If the user was authenticated and authorised, the result we want should be the first (and only, `key` is unique)
    // row we got back
    return {
        banner: banner[0]?.value,
        contents: contents[0]?.value,
    };
}
