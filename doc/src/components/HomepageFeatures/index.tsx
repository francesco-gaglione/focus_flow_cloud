import type { ReactNode } from "react";
import clsx from "clsx";
import Heading from "@theme/Heading";
import styles from "./styles.module.css";

type FeatureItem = {
    title: string;
    emoji: string;
    description: ReactNode;
};

const FeatureList: FeatureItem[] = [
    {
        title: 'Focus Sessions',
        emoji: '⏱️',
        description: (
            <>
                Manage work and break intervals efficiently. Track your productivity
                and maintain focus with our Pomodoro technique implementation.
            </>
        ),
    },
    {
        title: 'Real-time Sync',
        emoji: '⚡',
        description: (
            <>
                Synchronize your state across multiple devices using WebSockets.
                Stay updated instantly on your mobile and desktop.
            </>
        ),
    },
    {
        title: 'Task Management',
        emoji: '✅',
        description: (
            <>
                Create, organize, and color-code your to-dos. Analyze your productivity
                patterns with detailed statistics.
            </>
        ),
    },
];

function Feature({ title, emoji, description }: FeatureItem) {
    return (
        <div className={clsx('col col--4')}>
            <div className="feature-card text--center">
                <span className="feature-emoji" role="img" aria-label={title}>{emoji}</span>
                <Heading as="h3">{title}</Heading>
                <p>{description}</p>
            </div>
        </div>
    );
}

export default function HomepageFeatures(): ReactNode {
    return (
        <section className={styles.features}>
            <div className="container">
                <div className="row">
                    {FeatureList.map((props, idx) => (
                        <Feature key={idx} {...props} />
                    ))}
                </div>
            </div>
        </section>
    );
}
