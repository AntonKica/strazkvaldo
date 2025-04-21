import { SVC_USER_REPEATED_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    let repeated_activities_result = await fetch(SVC_USER_REPEATED_ACTIVITY.LIST()).then(response => {
        return response.json()
    });
    return {
        repeated_activities: repeated_activities_result.repeated_activities,
    };
};