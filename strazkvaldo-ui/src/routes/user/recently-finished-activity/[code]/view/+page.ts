import { SVC_USER_RECENTLY_FINISHED_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
    let recently_finished_activty_result = await fetch(SVC_USER_RECENTLY_FINISHED_ACTIVITY.GET(params.code)).then(response => {
        return response.json();
    });

    return {
        ... recently_finished_activty_result.data,
    };
};