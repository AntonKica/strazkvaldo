import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		repeatedActivities: [
            {
                id:0,
                name: "komunálny odpad",
                activityType: "garbageDisposal",
                criticalityType: "low",
                duration: 300,
                description: "vynášanie komunálneho odpadu do koša",
                periodicity: "week",
                startDate: "2025-03-24",
                endDate: "9999-01-01"
            },
            {
                id:1,
                name: "kuchynský odpad",
                activityType: "garbageDisposal",
                criticalityType: "low",
                duration: 300,
                description: "vynášanie kuchynského odpadu do koša",
                periodicity: "week",
                startDate: "2025-03-25",
                endDate: "9999-01-01"
            },
            {
                id:2,
                name: "triedený odpad",
                activityType: "garbageDisposal",
                criticalityType: "low",
                duration: 300,
                description: "vynášanie plastov, papierov a skla do koša",
                periodicity: "week",
                startDate: "2025-03-26",
                endDate: "9999-01-01"
            },
            {
                id:3,
                name: "pranie oblečenia",
                activityType: "washing",
                criticalityType: "normal",
                duration: 7200,
                description: "pranie oblečenia",
                periodicity: "month",
                startDate: "0001-01-01",
                endDate: "9999-01-01"
            }
        ],
		oneTimeActivities: [
            {
                id:4,
                name: "umyť topánky z blata",
                activityType: "washing",
                criticalityType: "high",
                duration: 900,
                description: "zvnútra sú zašpinené biele Adidas",
                date: "2025-03-23"
            }
       ]
	};
};