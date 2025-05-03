import { SVC_ENUM_APP_USER_ROLE} from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let user_role = await fetch(SVC_ENUM_APP_USER_ROLE.LIST()).then(response => {
        return response.json();
    });


    return {
        app_user_role: user_role.enum_values
    };
};