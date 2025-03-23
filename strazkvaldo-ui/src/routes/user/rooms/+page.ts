import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		rooms: [
            {
                name: "Obývacia izba",
                type: "livingroom",
                description: "rád tu pozeram telku"
            },
            {
                name: "Spálňa",
                type: "bedroom",
                description: "rád tu spím"
            }
        ]
	};
};