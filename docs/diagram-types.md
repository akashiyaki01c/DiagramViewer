# diagram-types

## 語句定義
* 実路線(`Route`) 物理的に存在する路線を表す。(実路線の途中に分岐線が挟まらないようにすること。)
* 運転系統(`Line`) 仮想的に存在する路線を表す。実際に旅客が認識するような路線のこと。(大阪環状線ー阪和線など)

## 時刻表を定義する上でのデータ構造
* `RootFile` ルートとなるファイル。ファイル保存の単位。
	* `FileVersion` ファイルのバージョンを表す。
	* `StationMap` 駅の集合を表す
		* `Station` 駅を表す
			* `StationId` 駅のIDを表す
	* `RouteMap` 実路線の集合を表す
		* `Route` 実路線を表す
			* `RouteId` 実路線のIDを表す
	* `LineMap` 運転系統の集合を表す
		* `Line` 運転系統を表す
			* `LineId` 運転系統のIDを表す
			* `RouteRefDirection` 実路線の運転方向を表す
	* `TrainTypeMap` 列車種別の集合を表す
		* `TrainType` 列車種別を表す
			* `TrainTypeId` 列車種別のIDを表す
	* `Diagram` ダイヤグラムを表す
		* `Train` 列車を表す
			* `Direction` 列車の運転方向を表す
			* `StationTime` 駅時刻を表す
				* `Time` 時刻を表す
				* `StopType` 停車種別を表す		
	* `Settings` 設定を表す
		* `DiagramDisplaySettings` ダイヤグラムの表示設定