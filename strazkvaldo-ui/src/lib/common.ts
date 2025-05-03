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

export function timestamp_to_string(datetime_rfc3339: string): string {
    return new Date(Date.parse(datetime_rfc3339)).toLocaleString("sk-SK");
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
export function day_of_week_to_string(day_of_week: Number): string {
    switch (day_of_week) {
        case 1: return "pondelok";
        case 2: return "utorok";
        case 3: return "streda";
        case 4: return "štvrtok";
        case 5: return "piatok";
        case 6: return "sobota";
        case 7: return "nedeľa";
        default: return "neznámy deň";
    }
}