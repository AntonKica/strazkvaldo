// place files you want to import through the `$lib` alias in this folder.
export function app_user_role_code_to_string(app_user_role_code: number): string {
    switch(app_user_role_code) {
        case 0: return "adminstrátor";
        case 1: return "používateľ";
        default: return "neznáma rola";
    };
}
export function app_user_role_to_string(app_user_role: AppUserRole): string {
    switch(app_user_role) {
        case AppUserRole.Administrator: return "adminstrátor";
        case AppUserRole.User: return "používateľ";
        default: return "neznáma rola";
    };
}

export enum AppUserRole {
    Unsigned='unknon',
    Administrator='admin',
    User='user',
}
export function app_user_role_from_int(appUserRole: string): AppUserRole {
    switch(appUserRole) {
        case 'admin': return AppUserRole.Administrator;
        case 'user': return AppUserRole.User;
        default: return AppUserRole.Unsigned;
    };
}
export function duration_in_seconds_to_string(duration_in_seconds: number): string {
    let hours = to_hours(duration_in_seconds);
    let minutes = to_minutes(duration_in_seconds);

    let res = "";
    if(hours > 0) {
        res += `${hours} hod. `;
    }
    if(minutes > 0) {
        res += `${minutes} min. `;
    }
    
    return res.trim();
}

export function datetime_rfc3339_to_string(datetime_rfc3339: string): string {
    let date = new Date(Date.parse(datetime_rfc3339));
    let res = `${date.getDate()}.${date.getMonth() + 1}.${date.getFullYear()}`;
    return res;
}
export function to_html_date(datetime_rfc3339: string): string {
    let date = new Date(Date.parse(datetime_rfc3339));
    //day += 1;
    let res = `${date.getFullYear()}-${(date.getMonth() + 1).toString().padStart(2, "0")}-${(date.getDate()).toString().padStart(2, "0")}`;
    return res;
}
export function from_html_date(html_date: string): string {
    let [year, month, day ] = html_date.split("-").map(Number);

    return new Date(year, month - 1, day).toISOString();
}
export function to_hours(duration_in_seconds: number): number {
    return duration_in_seconds / 3600 >> 0;
}
export function to_minutes(duration_in_seconds: number): number {
    return (duration_in_seconds % 3600) / 60 >> 0;
}
export function to_duration_in_seconds(minutes: number, hours: number): number {
    return minutes * 60 + hours * 3600;
}