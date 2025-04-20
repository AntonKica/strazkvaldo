import { SVC_ADMIN_APP_USER, SVC_ENUM_APP_USER_ROLE} from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let user = await fetch(SVC_ADMIN_APP_USER.GET(params.code)).then(response => {
        return response.json();
    });
    let user_role = await fetch(SVC_ENUM_APP_USER_ROLE.LIST()).then(response => {
        return response.json();
    });


    return {
        user: user.data.user,
        app_user_role: user_role.enum_values
    };
};