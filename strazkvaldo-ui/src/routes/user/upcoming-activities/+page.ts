import type { PageLoad } from './$types';
import { SVC_USER_UPCOMING_ACTIVITY } from '$lib/serviceRoutes';

export const load: PageLoad = async ({ params }) => {
    let upcoming_activities_result = await fetch(SVC_USER_UPCOMING_ACTIVITY.LIST()).then(response => {
        return response.json()
    });
    return {
        upcoming_activities: upcoming_activities_result.upcoming_activities,
    };
};