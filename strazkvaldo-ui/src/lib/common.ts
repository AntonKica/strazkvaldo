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
    Administrator=0,
    User=1,
}