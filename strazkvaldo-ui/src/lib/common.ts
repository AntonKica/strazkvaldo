// place files you want to import through the `$lib` alias in this folder.
export enum AppUserRole {
    Unsigned='unknon',
    Admin='Admin',
    User='User',
}
export function app_user_role_from_string(appUserRole: string): AppUserRole {
    switch(appUserRole) {
        case 'Admin': return AppUserRole.Admin;
        case 'User': return AppUserRole.User;
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
    let res = `${date.getFullYear()}-${(date.getMonth() + 1).toString().padStart(2, "0")}-${(date.getDate()).toString().padStart(2, "0")}`;
    return res;
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