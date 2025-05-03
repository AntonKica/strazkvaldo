import { SVC_USER_RECENTLY_FINISHED_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './recent/$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const finished_activities_result = await fetch(SVC_USER_RECENTLY_FINISHED_ACTIVITY.LIST()).then(response => {
        return response.json()
    });
    return {
        ... finished_activities_result,
    };
};