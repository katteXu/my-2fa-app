import { useEffect, useState } from "react";

type IProps = {
  width?: number;
  border?: number;
  time?: number;
};
const Index: React.FC<IProps> = (props) => {
  const { width = 50, border = 5, time = 10 } = props;
  const [animation, setAnimation] = useState(true);
  const round = width / 2;

  useEffect(() => {
    timer();
  }, [time]);

  const timer = () => {
    var circle = document.querySelectorAll("circle")[1];
    if (circle) {
      var percent = (30 - time) / 30,
        perimeter = Math.PI * 2 * (round - border / 2);
      circle.setAttribute(
        "stroke-dasharray",
        perimeter * percent + " " + perimeter * (1 - percent)
      );
      if (time === 30) {
        setAnimation(false);
      } else if (time === 29) {
        setAnimation(true);
      }
    }
  };
  return (
    <div
      className="flex justify-center] relative bg-white rounded-full"
      style={{ width: width, height: width }}
    >
      <svg width={width} height={width}>
        <circle
          cx={round}
          cy={round}
          r={round - border / 2}
          strokeWidth={border}
          stroke="#D1D3D7"
          fill="none"
        ></circle>
        <circle
          className={animation ? "loading" : ""}
          cx={round}
          cy={round}
          r={round - border / 2}
          strokeWidth={border}
          stroke="#f63"
          fill="none"
          transform={`matrix(0,-1,1,0,0,${width})`}
          strokeDasharray="0 1069"
        ></circle>
      </svg>
      <span className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-[#f63]">
        {time}
      </span>
    </div>
  );
};

export default Index;
