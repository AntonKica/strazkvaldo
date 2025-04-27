import { SVC_USER_APP_SETTINGS } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    let app_settings_result = await fetch(SVC_USER_APP_SETTINGS).then(response => {
        return response.json();
    });

    return {
        ... app_settings_result.data,
    };
};