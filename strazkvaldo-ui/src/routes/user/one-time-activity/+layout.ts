import { SVC_USER_ONE_TIME_ACTIVITY } from '$lib/serviceRoutes';
import type { PageLoad } from './$types';

export const load: PageLoad =  async ({ params }) => {
    // todo proxya https://vite.dev/config/server-options.html#server-proxy
    let one_time_activities = await fetch(SVC_USER_ONE_TIME_ACTIVITY.LIST()).then(response => {
        return response.json()
    }
    );
    return {
        /*
        repeated_activities: [
            {
                activity_code: 0,
                name: "komunálny odpad",
                activity_type: "garbageDisposal",
                criticality_type: "low",
                duration_in_seconds: 300,
                description: "vynášanie komunálneho odpadu do koša",
                periodicity: "week",
                startDate: "2025-03-24",
                endDate: "9999-01-01"
            },
            {
                activity_code: 1,
                name: "kuchynský odpad",
                activity_type: "garbageDisposal",
                criticality_type: "low",
                duration_in_seconds: 300,
                description: "vynášanie kuchynského odpadu do koša",
                periodicity: "week",
                startDate: "2025-03-25",
                endDate: "9999-01-01"
            },
            {
                activity_code: 2,
                name: "triedený odpad",
                activity_type: "garbageDisposal",
                criticality_type: "low",
                duration_in_seconds: 300,
                description: "vynášanie plastov, papierov a skla do koša",
                periodicity: "week",
                startDate: "2025-03-26",
                endDate: "9999-01-01"
            },
            {
                activity_code: 3,
                name: "pranie oblečenia",
                activity_type: "washing",
                criticality_type: "normal",
                duration_in_seconds: 7200,
                description: "pranie oblečenia",
                periodicity: "month",
                startDate: "0001-01-01",
                endDate: "9999-01-01"
            }
        ],
        */
        one_time_activities: one_time_activities.one_time_activities
    };
};