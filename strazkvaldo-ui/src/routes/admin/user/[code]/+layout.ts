import { SVC_ADMIN_APP_USER } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
    let user = await fetch(SVC_ADMIN_APP_USER.GET(params.code)).then(response => {
        return response.json();
    });

    return {
        user: user.data.user,
        roles: [0, 1]
    };
};