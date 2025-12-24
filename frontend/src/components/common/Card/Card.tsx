import React from 'react';
import styles from './Card.module.css';

export interface CardProps {
  children: React.ReactNode;
  className?: string;
  hoverable?: boolean;
  selected?: boolean;
  onClick?: () => void;
  style?: React.CSSProperties;
}

export const Card: React.FC<CardProps> = ({
  children,
  className = '',
  hoverable = false,
  selected = false,
  onClick,
  style,
}) => {
  const classes = [
    styles.card,
    hoverable && styles.hoverable,
    selected && styles.selected,
    onClick && styles.clickable,
    className,
  ]
    .filter(Boolean)
    .join(' ');

  const Component = onClick ? 'button' : 'div';

  return (
    <Component className={classes} onClick={onClick} type={onClick ? 'button' : undefined} style={style}>
      {children}
    </Component>
  );
};
