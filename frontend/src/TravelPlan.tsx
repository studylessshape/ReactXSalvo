import { useState } from "react";
import { initialTravelPlan } from "./places";
import { Button } from "antd";

function PlaceTree({ id, parentId, placesById, onComplete }: {
    id: number;
    parentId: number;
    placesById: typeof initialTravelPlan;
    onComplete: (parentId: number, childId: number) => void;
}) {
    const place = placesById.find((p) => p.id == id);
    if (place === undefined) {
        return <></>;
    }
    const childIds = place.childIds;

    return (
        <>
            <li>
                {place.title}
                <Button
                    onClick={() => {
                        onComplete(parentId, id);
                    }}
                >
                    Check complete
                </Button>
                {childIds.length > 0 && (
                    <ol>
                        {childIds.map((childId) => (
                            <PlaceTree
                                key={childId}
                                parentId={id}
                                id={childId}
                                placesById={placesById}
                                onComplete={onComplete}
                            />
                        ))}
                    </ol>
                )}
            </li>
        </>
    );
}

export default function TravelPlan() {
    const [plan, setPlan] = useState(initialTravelPlan);

    function handleComplete(parentId: number, childId: number) {
        function pushChildIds(id: number): number[] {
            const child = plan.find((p) => p.id == id);

            let childIds: number[] = [id];
            if (child != undefined) {
                child.childIds.forEach((id) => {
                    childIds = [...childIds, ...pushChildIds(id)];
                });
            }

            return childIds;
        }

        const childIds = pushChildIds(childId);

        setPlan(() => {
            let newPlan = plan.filter(({ id }) => !childIds.includes(id));

            newPlan.map((p) => {
                if (p.id == parentId) {
                    p.childIds = p.childIds.filter(
                        (id) => !childIds.includes(id),
                    );
                }
                return p;
            });

            return newPlan;
        });
    }

    const root = plan[0];
    const planetIds = root.childIds;
    return (
        <>
            <h2>Places to visit</h2>
            <ol>
                {planetIds.map((id) => (
                    <PlaceTree
                        key={id}
                        id={id}
                        parentId={0}
                        placesById={plan}
                        onComplete={handleComplete}
                    />
                ))}
            </ol>
        </>
    );
}
